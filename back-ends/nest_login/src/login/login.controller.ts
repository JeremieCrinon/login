import { Controller, Get, Post, Body, HttpCode, UseGuards } from '@nestjs/common';
import { LoginService } from './login.service';
import { LoginDto } from './dto/login.dto';
import { ModifyNewAccountDto } from './dto/modify-new-account.dto';
import { VerifyEmailDto } from './dto/verify-email.dto';
import { ForgotPasswordDto } from './dto/forgot-password.dto';
import { RequestUser } from 'src/auth/decorators/request-user.decorator';
import { Role, User } from 'src/user/entities/user.entity';
import { AuthGuard } from 'src/auth/guards/auth.guard';
import { RequiredRole } from 'src/auth/decorators/roles.decorator';
import { ResetPasswordDto } from './dto/reset-password.dto';
import { EditEmailDto } from './dto/edit-email.dto';
import { EditPasswordDto } from './dto/edit-password.dto';

@Controller('/')
export class LoginController {
  constructor(private readonly loginService: LoginService) { }

  @Post('login')
  @HttpCode(200)
  login(@Body() loginDto: LoginDto) {
    return this.loginService.login(loginDto);
  }

  @Get('user-infos')
  @UseGuards(AuthGuard)
  @HttpCode(200)
  userInfos(
    @RequestUser() user: User
  ) {
    return this.loginService.userInfos(user);
  }

  @Post('modify-new-account/:lang')
  @UseGuards(AuthGuard)
  @RequiredRole(Role.NEW_ACCOUNT)
  @HttpCode(200)
  modifyNewAccount(
    @Body() modifyNewAccountDto: ModifyNewAccountDto,
    @RequestUser() user: User
  ) {
    return this.loginService.modifyNewAccount(modifyNewAccountDto, user);
  }

  @Post('verify-email')
  @UseGuards(AuthGuard)
  @RequiredRole(Role.UNVERIFIED_EMAIL)
  @HttpCode(200)
  verifyEmail(
    @Body() verifyEmailDto: VerifyEmailDto,
    @RequestUser() user: User
  ) {
    return this.loginService.verifyEmail(verifyEmailDto, user);
  }

  @Post('forgot-password/:lang')
  @HttpCode(200)
  forgotPassword(
    @Body() forgotPasswordDto: ForgotPasswordDto
  ) {
    return this.loginService.forgotPassword(forgotPasswordDto);
  }

  @Post('reset-password')
  @HttpCode(200)
  resetPassword(
    @Body() resetPasswordDto: ResetPasswordDto
  ) {
    return this.loginService.resetPassword(resetPasswordDto);
  }

  @Post('edit-email/:lang')
  @UseGuards(AuthGuard)
  @HttpCode(200)
  editEmail(
    @Body() editEmailDto: EditEmailDto,
    @RequestUser() user: User
  ) {
    return this.loginService.editEmail(editEmailDto, user);
  }

  @Post('edit-password')
  @UseGuards(AuthGuard)
  @RequiredRole(Role.USER)
  @HttpCode(200)
  editPassword(
    @Body() editPasswordDto: EditPasswordDto,
    @RequestUser() user: User
  ) {
    return this.loginService.editPassword(editPasswordDto, user);
  }
}
