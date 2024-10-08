import { Injectable } from "@nestjs/common";
import { InjectModel } from "@nestjs/mongoose";
import { Model } from "mongoose";
import { MONGO_CONNECTION_NAME } from "#constants";
import { Comment } from "./comment.schema";

@Injectable()
export class CommentService {
    constructor(@InjectModel(Comment.name, MONGO_CONNECTION_NAME) private commentModel: Model<Comment>) {}

    async create(comment: Comment) {
        return await this.commentModel.create(comment);
    }

    async remove(id: string) {
        return await this.commentModel.findByIdAndDelete(id).exec();
    }

    async update(comment: Comment) {
        return await this.commentModel.findByIdAndUpdate(comment._id, comment, { new: true }).exec();
    }

    async find(count: number) {
        return await this.commentModel
            .find()
            .select(["-email", "-password"])
            .sort({ createdAt: -1 })
            .limit(count)
            .exec();
    }

    async findById(id: string) {
        return await this.commentModel.findById(id).select(["-email", "-password"]).exec();
    }

    async findByPostSlug(postSlug: string): Promise<(Omit<Comment, "parentCommentId"> & { replies: Comment[] })[]> {
        const comments = await this.commentModel
            .find({ postSlug: { $eq: postSlug } })
            .select(["-email", "-password"])
            .sort({ createdAt: -1 })
            .exec();
        const parentComments = comments.filter((comment) => !comment.parentCommentId);
        const nestedCommentsWithReplies = parentComments.map((comment) => {
            const replies = comments.filter((reply) => reply.parentCommentId === comment.id).reverse();

            return { ...comment.toJSON(), replies };
        });

        return nestedCommentsWithReplies;
    }
}
