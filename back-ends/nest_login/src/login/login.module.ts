import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { LoginService } from './login.service';
import { LoginController } from './login.controller';
import { User } from '../user/entities/user.entity';
import { EmailModule } from 'src/email/email.module';
import { SharedModule } from 'src/shared.module';
import { EmailVerificationHelper } from './helpers/email-verification.helper';

@Module({
  imports: [
    TypeOrmModule.forFeature([User]),
    EmailModule,
    SharedModule
  ],
  controllers: [LoginController],
  providers: [LoginService, EmailVerificationHelper],
  exports: [EmailVerificationHelper],
})
export class LoginModule { }
