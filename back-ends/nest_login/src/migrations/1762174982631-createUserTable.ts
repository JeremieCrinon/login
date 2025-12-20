import { MigrationInterface, QueryRunner } from "typeorm";
import * as bcrypt from 'bcrypt';

export class CreateUserTable1762174982631 implements MigrationInterface {
    name = 'CreateUserTable1762174982631'

    public async up(queryRunner: QueryRunner): Promise<void> {
        await queryRunner.query(`
            CREATE TYPE "public"."user_role_enum" AS ENUM(
                'admin',
                'edit_users',
                'user',
                'unverified_email',
                'new_account'
            )
        `);
        await queryRunner.query(`
            CREATE TABLE "user" (
                "id" SERIAL NOT NULL,
                "email" character varying NOT NULL,
                "password" character varying NOT NULL,
                "role" "public"."user_role_enum" array NOT NULL,
                "emailVerificationCode" character varying,
                "passwordResetCode" character varying,
                "createdAt" TIMESTAMP NOT NULL DEFAULT now(),
                "updatedAt" TIMESTAMP NOT NULL DEFAULT now(),
                CONSTRAINT "PK_cace4a159ff9f2512dd42373760" PRIMARY KEY ("id")
            )
        `);

        const firstUserEmail = "email@mail.com";
        const firstUserPassword = "Admin12345@";
        const firstUserHashedPassword = await bcrypt.hash(firstUserPassword, 10);
        const firstUserRoles = ['admin', 'new_account'];

        await queryRunner.query(`
            INSERT INTO "user" ("email", "password", "role")
            VALUES ($1, $2, $3)
        `, [firstUserEmail, firstUserHashedPassword, firstUserRoles]);
    }

    public async down(queryRunner: QueryRunner): Promise<void> {
        await queryRunner.query(`
            DROP TABLE "user"
        `);
        await queryRunner.query(`
            DROP TYPE "public"."user_role_enum"
        `);
    }

}
