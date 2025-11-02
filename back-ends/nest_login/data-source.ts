import 'dotenv/config';
import { DataSource } from 'typeorm';
import { User } from './src/user/entities/user.entity';

export const AppDataSource = new DataSource({
  type: 'postgres',
  host: process.env.DB_HOST,
  port: Number(process.env.DB_PORT),
  username: process.env.DB_USER,
  password: process.env.DB_PASS,
  database: process.env.DB_NAME,
  entities: [User],
  migrations: ['src/migrations/*.ts'],
  synchronize: false,
});
