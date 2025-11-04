import 'dotenv/config';
import { DataSource } from 'typeorm';
import { User } from './src/user/entities/user.entity';
import { env } from 'src/env';

export const AppDataSource = new DataSource({
  type: 'postgres',
  host: env("DB_HOST"),
  port: Number(env("DB_PORT")),
  username: env("DB_USER"),
  password: env("DB_PASS"),
  database: env("DB_NAME"),
  entities: [User],
  migrations: ['src/migrations/*.ts'],
  synchronize: false,
});
