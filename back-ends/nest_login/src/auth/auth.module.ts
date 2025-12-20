import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { User } from '../user/entities/user.entity';
import { SharedModule } from 'src/shared.module';
import { AuthGuard } from './guards/auth.guard';

@Module({
  imports: [
    TypeOrmModule.forFeature([User]),
    SharedModule
  ],
  providers: [AuthGuard],
  exports: [AuthGuard],
})
export class AuthModule { }
