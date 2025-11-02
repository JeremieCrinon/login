import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { UserModule } from './user/user.module';
import { User } from './user/entities/user.entity';

@Module({
  imports: [
    TypeOrmModule.forRoot({
      type: 'postgres',
      host: 'localhost',
      port: 5432,
      username: 'root',
      password: 'root',
      database: 'nest_login',
      entities: [User],
      migrations: ['src/migrations/*.ts'],
      synchronize: false,
    }),
    UserModule
  ],
  // controllers: [],
  // providers: [],
})
export class AppModule { }
