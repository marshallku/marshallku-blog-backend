import { z } from "zod";

const koreanRegex = /[ㄱ-ㅎ가-힣]/;

export const commentAddRequestSchema = z.object({
    name: z.string().min(2, "이름을 입력해 주세요.").max(20, "이름은 20자 이내로 입력해 주세요."),
    postSlug: z.string().min(1, "올바르지 않은 요청입니다."),
    body: z.string().min(2, "댓글을 입력해 주세요.").regex(koreanRegex, "한글을 입력해 주세요."),
    url: z.string().url("올바른 URL을 입력해 주세요.").or(z.undefined()).or(z.string().length(0)),
});

export const commentModifyRequestSchema = z.object({
    _id: z.string().min(1, "올바르지 않은 요청입니다."),
    name: z.string().max(20, "이름은 20자 이내로 입력해 주세요."),
    body: z.string().regex(koreanRegex, "한글을 입력해 주세요."),
    url: z.string().url("올바른 URL을 입력해 주세요.").or(z.undefined()).or(z.string().length(0)),
});
