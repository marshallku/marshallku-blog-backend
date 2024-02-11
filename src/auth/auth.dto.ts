import { ApiProperty } from "@nestjs/swagger";

export class SignInRequest {
    @ApiProperty({ description: "Name of a user" })
    name: string;

    @ApiProperty({ description: "Password of a user" })
    password: string;
}

export class SignInResponse {
    @ApiProperty()
    token: string;
}
