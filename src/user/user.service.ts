import { FilterQuery, Model } from "mongoose";
import { Injectable } from "@nestjs/common";
import { InjectModel } from "@nestjs/mongoose";
import { MONGO_CONNECTION_NAME } from "#constants";
import { User } from "./user.schema";

@Injectable()
export class UserService {
    constructor(@InjectModel(User.name, MONGO_CONNECTION_NAME) private userModel: Model<User>) {}

    async create(createUserDto: Pick<User, "name" | "password">) {
        const createdUser = new this.userModel(createUserDto);
        return createdUser.save();
    }

    async findOne(name: string): Promise<User | undefined> {
        return this.userModel.findOne({ name }).exec();
    }

    async findOneBy(query: FilterQuery<User>): Promise<User | undefined> {
        return this.userModel.findOne(query).exec();
    }

    async findAll(): Promise<User[]> {
        return this.userModel.find({}, { password: 0 }).exec();
    }
}
