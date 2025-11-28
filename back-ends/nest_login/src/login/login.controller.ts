import { Controller, Get, Post, Body, HttpCode, UseGuards } from '@nestjs/common';
import { LoginService } from './login.service';
import { LoginDto } from './dto/login.dto';
import { ModifyNewAccountDto } from './dto/modify-new-account.dto';
import { VerifyEmailDto } from './dto/verify-email.dto';
import { RequestUser } from 'src/auth/decorators/request-user.decorator';
import { Role, User } from 'src/user/entities/user.entity';
import { AuthGuard } from 'src/auth/guards/auth.guard';
import { RequiredRole } from 'src/auth/decorators/roles.decorator';

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
}
