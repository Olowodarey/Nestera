import { Global, Module } from '@nestjs/common';
import { StellarService } from './stellar.service';
import { SavingsService } from './savings.service';
import { BlockchainController } from './blockchain.controller';

@Global()
@Module({
  controllers: [BlockchainController],
  providers: [StellarService, SavingsService],
  exports: [StellarService, SavingsService],
})
export class BlockchainModule {}
