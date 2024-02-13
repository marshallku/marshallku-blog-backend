import { Module } from "@nestjs/common";
import * as Joi from "joi";
import { ConfigModule } from "@nestjs/config";
import { MongooseModule } from "@nestjs/mongoose";
import { MONGO_CONNECTION_NAME } from "#constants";
import { AppController } from "./app.controller";
import { AppService } from "./app.service";
import { AuthController } from "./auth/auth.controller";
import { AuthModule } from "./auth/auth.module";
import { UserController } from "./user/user.controller";
import { UserModule } from "./user/user.module";
import { CommentController } from "./comment/comment.controller";
import { CommentModule } from "./comment/comment.module";
import { ApmModule } from "./apm/apm.module";

@Module({
    imports: [
        ConfigModule.forRoot({
            isGlobal: true,
            envFilePath: `.env`,
            validationSchema: Joi.object({
                NODE_ENV: Joi.string().valid("development", "production", "test", "provision").default("development"),
                NODE_PORT: Joi.string().required(),
                MONGO_PORT: Joi.string().required(),
                MONGO_HOST: Joi.string().required(),
                MONGO_USERNAME: Joi.string().required(),
                MONGO_PASSWORD: Joi.string().required(),
                COOKIE_DOMAIN: Joi.string().required(),
            }),
        }),
        MongooseModule.forRoot(`mongodb://${process.env.MONGO_HOST}:${process.env.MONGO_PORT}/`, {
            dbName: "default",
            connectionName: MONGO_CONNECTION_NAME,
            auth: {
                username: process.env.MONGO_USERNAME,
                password: process.env.MONGO_PASSWORD,
            },
            authSource: "admin",
        }),
        AuthModule,
        UserModule,
        CommentModule,
        ApmModule,
    ],
    controllers: [AppController, AuthController, UserController, CommentController],
    providers: [AppService],
})
export class AppModule {}
