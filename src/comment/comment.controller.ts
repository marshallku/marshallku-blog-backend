import { BadRequestException, Body, Controller, Get, HttpCode, HttpStatus, Post, Query, Req } from "@nestjs/common";
import { ApiOperation, ApiTags } from "@nestjs/swagger";
import { z } from "zod";
import { Public } from "#utils";
import { UserRole } from "#constants";
import { JWTUser } from "#types";
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
    async createComment(@Req() req: { user: JWTUser }, @Body() comment: Comment) {
        try {
            commentAddRequestSchema.parse(comment);
        } catch (error) {
            console.error(error);

            if (error instanceof z.ZodError) {
                throw new BadRequestException(error.errors[0].message);
            }

            throw new BadRequestException("잘못된 요청입니다.");
        }

        if (req.user && req.user.role === UserRole.Root) {
            comment.byPostAuthor = true;
        }

        return await this.commentService.create(comment);
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
