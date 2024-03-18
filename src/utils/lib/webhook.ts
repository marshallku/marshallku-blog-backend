interface DiscordFieldData {
    name: string;
    value: string;
    inline: boolean;
}

export async function sendDiscordMessage(title: string, description: string, fields: DiscordFieldData[] = []) {
    if (!process.env.DISCORD_WEBHOOK_URL) {
        return;
    }

    try {
        await fetch(process.env.DISCORD_WEBHOOK_URL, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                embeds: {
                    type: "rich",
                    title,
                    description,
                    fields,
                },
            }),
        });
        return true;
    } catch (error) {
        console.error(error);
        return false;
    }
}
