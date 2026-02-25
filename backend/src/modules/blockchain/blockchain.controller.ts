import { Controller, Get, Param, Post } from '@nestjs/common';
import { ApiOperation, ApiParam, ApiResponse, ApiTags } from '@nestjs/swagger';
import { StellarService } from './stellar.service';
import { TransactionDto } from './dto/transaction.dto';

@ApiTags('Blockchain')
@Controller('blockchain')
export class BlockchainController {
  constructor(private readonly stellarService: StellarService) {}

  @Post('wallets/generate')
  @ApiOperation({ summary: 'Generate a new Stellar keypair' })
  generateWallet() {
    return this.stellarService.generateKeypair();
  }

  @Get('wallets/:publicKey/transactions')
  @ApiOperation({
    summary: 'Get recent on-chain transactions for a Stellar wallet',
  })
  @ApiParam({
    name: 'publicKey',
    description: 'The Stellar public key (starting with G) of the wallet',
    example: 'GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN',
  })
  @ApiResponse({
    status: 200,
    description: 'Array of recent transactions mapped to sanitized objects',
    type: [TransactionDto],
  })
  getWalletTransactions(
    @Param('publicKey') publicKey: string,
  ): Promise<TransactionDto[]> {
    return this.stellarService.getRecentTransactions(publicKey);
  }
}
