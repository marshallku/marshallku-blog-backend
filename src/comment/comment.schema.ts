import { Document, HydratedDocument } from "mongoose";
import { Prop, Schema, SchemaFactory } from "@nestjs/mongoose";

export type CommentDocument = HydratedDocument<Comment>;

@Schema({ timestamps: true })
export class Comment extends Document {
    @Prop({ required: true })
    name: string;

    @Prop({ required: true, index: true })
    postSlug: string;

    @Prop()
    password: string;

    @Prop()
    email: string;

    @Prop()
    url: string;

    @Prop()
    body: string;
}

export const CommentSchema = SchemaFactory.createForClass(Comment);
