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
import { UpdateUserDto } from './dto/update-user.dto';
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
  ) {}

  @Get('me')
  getMe(@CurrentUser() user: { id: string }) {
    return this.userService.findById(user.id);
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
}
