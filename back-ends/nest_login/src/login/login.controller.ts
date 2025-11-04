import { Controller, Post, Body } from '@nestjs/common';
import { LoginService } from './login.service';
import { LoginDto } from './dto/login.dto';

@Controller('/')
export class LoginController {
  constructor(private readonly loginService: LoginService) { }

  @Post('login')
  login(@Body() loginDto: LoginDto) {
    return this.loginService.login(loginDto);
  }
}
