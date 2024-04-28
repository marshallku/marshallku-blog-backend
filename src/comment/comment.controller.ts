import {
    BadRequestException,
    Body,
    Controller,
    Delete,
    Get,
    HttpCode,
    HttpStatus,
    Post,
    Query,
    Req,
} from "@nestjs/common";
import { ApiOperation, ApiTags } from "@nestjs/swagger";
import { z } from "zod";
import { Public, sendDiscordMessage } from "#utils";
import { User } from "#user/user.schema";
import { UserRole } from "#constants";
import { Comment } from "./comment.schema";
import { CommentService } from "./comment.service";
import { commentAddRequestSchema } from "./comment.validator";

@Controller("comment")
@ApiTags("Comment API")
export class CommentController {
    constructor(private commentService: CommentService) {}

    @HttpCode(HttpStatus.OK)
    @Post("create")
    @ApiOperation({ summary: "Create a comment" })
    @Public()
    async createComment(@Req() req: { user?: User }, @Body() comment: Comment) {
        try {
            commentAddRequestSchema.parse(comment);
        } catch (error) {
            console.error(error);

            if (error instanceof z.ZodError) {
                throw new BadRequestException(error.errors[0].message);
            }

            throw new BadRequestException("잘못된 요청입니다.");
        }

        comment.byPostAuthor = req.user && req.user.role === UserRole.Root;

        const createdComment = await this.commentService.create(comment);

        await sendDiscordMessage(
            "New comment added",
            `New comment added by ${createdComment.name} on ${createdComment.postSlug}`,
            [
                { name: "Name", value: createdComment.name, inline: true },
                { name: "Email", value: createdComment.email, inline: true },
                { name: "Content", value: createdComment.body, inline: false },
            ],
        );

        return createdComment;
    }

    @HttpCode(HttpStatus.OK)
    @Delete("delete")
    @ApiOperation({ summary: "Delete a comment" })
    async deleteComment(@Req() req: { user?: User }, @Query("id") id: string) {
        const comment = await this.commentService.findById(id);

        if (!comment) {
            throw new BadRequestException("Comment not found");
        }

        if (!req.user || req.user.role !== UserRole.Root) {
            throw new BadRequestException("You are not authorized to delete this comment");
        }

        return await this.commentService.remove(id);
    }

    @HttpCode(HttpStatus.OK)
    @Get("list")
    @ApiOperation({ summary: "Get all comments" })
    @Public()
    async getAllComments(@Query("postSlug") postSlug: string) {
        return await this.commentService.findByPostSlug(postSlug);
    }

    @HttpCode(HttpStatus.OK)
    @Get("recent")
    @ApiOperation({ summary: "Get recent comments" })
    @Public()
    async getRecentComments(@Query("count") count?: string) {
        const maxCount = 10;
        const defaultCount = 5;
        return await this.commentService.find(count ? Math.min(parseInt(count), maxCount) : defaultCount);
    }
}
