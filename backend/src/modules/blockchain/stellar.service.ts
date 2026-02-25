import { Injectable, Logger, OnModuleInit } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import {
  Asset,
  Horizon,
  Keypair,
  Networks,
  rpc,
  Transaction,
  xdr,
} from '@stellar/stellar-sdk';
import { TransactionDto } from './dto/transaction.dto';

@Injectable()
export class StellarService implements OnModuleInit {
  private readonly logger = new Logger(StellarService.name);
  private rpcServer: rpc.Server;
  private horizonServer: Horizon.Server;

  constructor(private configService: ConfigService) {
    const rpcUrl = this.configService.get<string>('stellar.rpcUrl') || '';
    const horizonUrl =
      this.configService.get<string>('stellar.horizonUrl') || '';

    this.rpcServer = new rpc.Server(rpcUrl);
    this.horizonServer = new Horizon.Server(horizonUrl);
  }

  onModuleInit() {
    this.logger.log('Stellar Service Initialized');
    const network = this.configService.get<string>('stellar.network');
    this.logger.log(`Target Network: ${network}`);
  }

  getRpcServer() {
    return this.rpcServer;
  }

  getHorizonServer() {
    return this.horizonServer;
  }

  getNetworkPassphrase(): string {
    const network = this.configService.get<string>('stellar.network');
    return network === 'mainnet' ? Networks.PUBLIC : Networks.TESTNET;
  }

  async getHealth() {
    try {
      const health = await this.rpcServer.getHealth();
      return health;
    } catch (error) {
      this.logger.error('Failed to get Stellar RPC health', error);
      throw error;
    }
  }

  // Placeholder for Soroban contract interaction
  async queryContract(contractId: string, method: string) {
    // Implementation for querying smart contracts
    this.logger.log(`Querying contract ${contractId}, method ${method}`);
    // return this.rpcServer.simulateTransaction(...)
    return Promise.resolve();
  }

  generateKeypair(): { publicKey: string; secretKey: string } {
    const keypair = Keypair.random();
    return {
      publicKey: keypair.publicKey(),
      secretKey: keypair.secret(),
    };
  }

  /**
   * Fetches recent transactions for a given Stellar public key from the
   * Horizon server and maps them into a sanitized TransactionDto array.
   *
   * @param publicKey - The Stellar G... public key of the account
   * @param limit     - Maximum number of transactions to return (default 10)
   * @returns         Array of sanitized TransactionDto objects
   */
  async getRecentTransactions(
    publicKey: string,
    limit = 10,
  ): Promise<TransactionDto[]> {
    try {
      const response = await this.horizonServer
        .transactions()
        .forAccount(publicKey)
        .limit(limit)
        .order('desc')
        .call();

      const transactions = response.records;

      const results = await Promise.all(
        transactions.map(async (tx) => {
          // Default token / amount in case operations cannot be fetched
          let token = 'XLM';
          let amount = '0';

          try {
            const opsResponse = await tx.operations();
            const ops = opsResponse.records;

            if (ops.length > 0) {
              const op = ops[0] as unknown as Record<string, unknown>;

              // Extract amount — present on payment, path_payment, etc.
              if (typeof op['amount'] === 'string') {
                amount = op['amount'] as string;
              }

              // Determine the asset / token type
              if (
                op['asset_type'] === 'native' ||
                op['asset'] instanceof Asset
              ) {
                token = 'XLM';
              } else if (
                typeof op['asset_code'] === 'string' &&
                op['asset_code']
              ) {
                token = op['asset_code'] as string;
              } else if (op['buying_asset_code']) {
                token = op['buying_asset_code'] as string;
              } else if (op['selling_asset_code']) {
                token = op['selling_asset_code'] as string;
              }
            }
          } catch (opError) {
            this.logger.warn(
              `Could not fetch operations for tx ${tx.hash}: ${(opError as Error).message}`,
            );
          }

          return {
            date: tx.created_at,
            amount,
            token,
            hash: tx.hash,
          } satisfies TransactionDto;
        }),
      );

      return results;
    } catch (error) {
      this.logger.error(
        `Failed to fetch transactions for ${publicKey}: ${(error as Error).message}`,
        error,
      );
      return [];
    }
  }
}
