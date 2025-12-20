import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { UserService } from './user.service';
import { UserController } from './user.controller';
import { User } from './entities/user.entity';
import { EmailModule } from 'src/email/email.module';
import { AuthModule } from 'src/auth/auth.module';
import { SharedModule } from 'src/shared.module';
import { LoginModule } from 'src/login/login.module';

@Module({
  imports: [
    TypeOrmModule.forFeature([User]),
    EmailModule,
    AuthModule,
    SharedModule,
    LoginModule
  ],
  controllers: [UserController],
  providers: [UserService],
})
export class UserModule { }
