import { Module } from "@nestjs/common";
import { MongooseModule } from "@nestjs/mongoose";
import { MONGO_CONNECTION_NAME } from "#constants";
import { Comment, CommentSchema } from "./comment.schema";
import { CommentService } from "./comment.service";
import { CommentController } from "./comment.controller";

@Module({
    imports: [MongooseModule.forFeature([{ name: Comment.name, schema: CommentSchema }], MONGO_CONNECTION_NAME)],
    providers: [CommentService],
    exports: [CommentService],
    controllers: [CommentController],
})
export class CommentModule {}
