import {
  Controller,
  Get,
  Patch,
  Delete,
  Param,
  Body,
  UseGuards,
  Request,
  Post,
  UseInterceptors,
  UploadedFile,
  ParseFilePipe,
  MaxFileSizeValidator,
  FileTypeValidator,
} from '@nestjs/common';
import { FileInterceptor } from '@nestjs/platform-express';
import { StorageService } from '../storage/storage.service';
import { UserService } from './user.service';
import { SavingsService } from '../blockchain/savings.service';
import { UpdateUserDto } from './dto/update-user.dto';
import { NetWorthDto } from './dto/net-worth.dto';
import { JwtAuthGuard } from '../../auth/guards/jwt-auth.guard';
import { CurrentUser } from '../../common/decorators/current-user.decorator';

import { FileValidator } from '@nestjs/common';

class ImageTypeValidator extends FileValidator {
  constructor() {
    super({});
  }

  isValid(file: Express.Multer.File): boolean {
    const allowedTypes = ['image/jpeg', 'image/png', 'image/webp'];
    return allowedTypes.includes(file.mimetype);
  }

  buildErrorMessage(): string {
    return 'Invalid file type. Only jpeg, png, and webp are allowed.';
  }
}

@Controller('users')
@UseGuards(JwtAuthGuard)
export class UserController {
  constructor(
    private readonly userService: UserService,
    private readonly storageService: StorageService,
    private readonly savingsService: SavingsService,
  ) {}

  @Get('me')
  getMe(@CurrentUser() user: { id: string }) {
    return this.userService.findById(user.id);
  }

  @Get('me/net-worth')
  async getNetWorth(@CurrentUser() user: { id: string }): Promise<NetWorthDto> {
    const userEntity = await this.userService.findById(user.id);
    
    // If user has no public key, return zero balances
    if (!userEntity.publicKey) {
      return this.createZeroNetWorthResponse();
    }

    // Fetch wallet and savings data in parallel
    const [walletBalance, savingsBalance] = await Promise.all([
      this.savingsService.getWalletBalance(userEntity.publicKey),
      this.savingsService.getUserSavingsBalance(userEntity.publicKey),
    ]);

    const totalSavings = savingsBalance.total;
    const totalNetWorth = walletBalance + totalSavings;

    // Calculate percentages
    const walletPercentage = totalNetWorth > 0 ? (walletBalance / totalNetWorth) * 100 : 0;
    const savingsPercentage = totalNetWorth > 0 ? (totalSavings / totalNetWorth) * 100 : 0;

    return {
      walletBalance,
      savingsFlexible: savingsBalance.flexible,
      savingsLocked: savingsBalance.locked,
      totalSavings,
      totalNetWorth,
      balanceBreakdown: {
        wallet: {
          amount: walletBalance,
          percentage: walletPercentage,
        },
        savings: {
          amount: totalSavings,
          percentage: savingsPercentage,
          flexibleAmount: savingsBalance.flexible,
          lockedAmount: savingsBalance.locked,
        },
      },
    };
  }

  @Get(':id')
  findOne(@Param('id') id: string) {
    return this.userService.findById(id);
  }

  @Patch('me')
  updateMe(@CurrentUser() user: { id: string }, @Body() dto: UpdateUserDto) {
    return this.userService.update(user.id, dto);
  }

  @Delete('me')
  deleteMe(@CurrentUser() user: { id: string }) {
    return this.userService.remove(user.id);
  }

  @Post('avatar')
  @UseInterceptors(FileInterceptor('file'))
  async uploadAvatar(
    @CurrentUser() user: { id: string },
    @UploadedFile(
      new ParseFilePipe({
        validators: [
          new MaxFileSizeValidator({ maxSize: 1024 * 1024 * 5 }), // 5MB
          new ImageTypeValidator(),
        ],
      }),
    )
    file: Express.Multer.File,
  ) {
    const avatarUrl = await this.storageService.saveFile(file);
    return this.userService.updateAvatar(user.id, avatarUrl);
  }

  private createZeroNetWorthResponse(): NetWorthDto {
    return {
      walletBalance: 0,
      savingsFlexible: 0,
      savingsLocked: 0,
      totalSavings: 0,
      totalNetWorth: 0,
      balanceBreakdown: {
        wallet: {
          amount: 0,
          percentage: 0,
        },
        savings: {
          amount: 0,
          percentage: 0,
          flexibleAmount: 0,
          lockedAmount: 0,
        },
      },
    };
  }
}
