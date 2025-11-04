import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { TypeOrmModule } from '@nestjs/typeorm';
import { UserModule } from './user/user.module';
import { User } from './user/entities/user.entity';
import { EmailModule } from './email/email.module';
import { LoginModule } from './login/login.module';
import { SharedModule } from './shared.module';
import { JwtModule } from '@nestjs/jwt';
import { env } from './env';

@Module({
  imports: [
    ConfigModule.forRoot(),
    TypeOrmModule.forRoot({
      type: 'postgres',
      host: env("DB_HOST"),
      port: Number(env("DB_PORT")),
      username: env("DB_USER"),
      password: env("DB_PASS"),
      database: env("DB_NAME"),
      entities: [User],
      // migrations: ['src/migrations/*.ts'],
      synchronize: false
    }),
    SharedModule,
    UserModule,
    EmailModule,
    LoginModule,
    JwtModule,
  ],
})
export class AppModule { }
