import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { LoginService } from './login.service';
import { LoginController } from './login.controller';
import { User } from '../user/entities/user.entity';
import { EmailModule } from 'src/email/email.module';
import { SharedModule } from 'src/shared.module';

@Module({
  imports: [
    TypeOrmModule.forFeature([User]),
    EmailModule,
    SharedModule
  ],
  controllers: [LoginController],
  providers: [LoginService],
})
export class LoginModule { }
