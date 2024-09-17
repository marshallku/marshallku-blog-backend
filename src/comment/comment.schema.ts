import { Document, HydratedDocument, Types } from "mongoose";
import { Prop, Schema, SchemaFactory } from "@nestjs/mongoose";

export type CommentDocument = HydratedDocument<Comment>;

@Schema({ collection: "comment", timestamps: true })
export class Comment extends Document<string> {
    @Prop({ default: "익명" })
    name: string;

    @Prop({ required: true, index: true })
    postSlug: string;

    @Prop({ default: false })
    byPostAuthor: boolean;

    @Prop({ default: "" })
    password: string;

    @Prop({ default: "" })
    email: string;

    @Prop({ default: "" })
    url: string;

    @Prop({ required: true })
    body: string;

    @Prop({ type: Types.ObjectId, ref: "comment" })
    parentCommentId: Types.ObjectId;
}

export const CommentSchema = SchemaFactory.createForClass(Comment);
