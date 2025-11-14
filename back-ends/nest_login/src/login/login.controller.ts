import { Controller, Post, Body, HttpCode } from '@nestjs/common';
import { LoginService } from './login.service';
import { LoginDto } from './dto/login.dto';

@Controller('/')
export class LoginController {
  constructor(private readonly loginService: LoginService) { }

  @Post('login')
  @HttpCode(200)
  login(@Body() loginDto: LoginDto) {
    return this.loginService.login(loginDto);
  }
}
