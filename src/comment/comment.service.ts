import { Injectable } from "@nestjs/common";
import { InjectModel } from "@nestjs/mongoose";
import { Model } from "mongoose";
import { Comment } from "./comment.schema";
import { MONGO_CONNECTION_NAME } from "#constants";

@Injectable()
export class CommentService {
    constructor(@InjectModel(Comment.name, MONGO_CONNECTION_NAME) private commentModel: Model<Comment>) {}

    async create(comment: Comment) {
        return await this.commentModel.create(comment);
    }

    async remove(id: string) {
        return await this.commentModel.findByIdAndDelete(id).exec();
    }

    async find(count: number) {
        return await this.commentModel.find().sort({ createdAt: -1 }).limit(count).exec();
    }

    async findByPostSlug(postSlug: string) {
        return await this.commentModel.find({ postSlug }).exec();
    }
}
