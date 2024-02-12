import { z } from "zod";

export const commentAddRequestSchema = z.object({
    name: z.string().min(2, "이름을 입력해 주세요.").max(20, "이름은 20자 이내로 입력해 주세요."),
    postSlug: z.string().min(1, "올바르지 않은 요청입니다."),
    body: z
        .string()
        .min(2, "댓글을 입력해 주세요.")
        .regex(/[ㄱ-ㅎ가-힣]/, "한글을 입력해 주세요."),
});
