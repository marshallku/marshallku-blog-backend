import { inspect } from "node:util";
import { CallHandler, ExecutionContext, Injectable, NestInterceptor } from "@nestjs/common";
import { Observable, tap } from "rxjs";
import newrelic from "newrelic";

@Injectable()
export class ApmInterceptor implements NestInterceptor {
    intercept(context: ExecutionContext, next: CallHandler): Observable<any> {
        console.log(`Parent Interceptor before: ${inspect(context.getHandler().name)}`);
        return newrelic.startBackgroundTransaction(context.getClass().name, "APM", () => {
            const transaction = newrelic.getTransaction();
            return next.handle().pipe(
                tap(() => {
                    console.log(`Parent Interceptor after: ${inspect(context.getHandler().name)}`);
                    return transaction.end();
                }),
            );
        });
    }
}
