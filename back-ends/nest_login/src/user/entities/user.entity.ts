import { Entity, Column, PrimaryGeneratedColumn, CreateDateColumn, UpdateDateColumn } from 'typeorm';

export enum Role {
  ADMIN = "admin",
  EDIT_USERS = "edit_users",
  USER = "user",
  UNVERIFIED_EMAIL = "unverified_email",
  NEW_ACCOUNT = "new_account"
}

@Entity()
export class User {
  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  email: string;

  @Column()
  password: string;

  @Column({
    type: "enum",
    enum: Role,
    array: true,
    nullable: false
  })
  role: Role[];

  @Column({ type: 'varchar', nullable: true })
  emailVerificationCode: string | null;

  @Column({ type: 'varchar', nullable: true })
  passwordResetCode: string | null;

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
