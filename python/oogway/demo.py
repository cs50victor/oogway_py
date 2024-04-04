import asyncio
import oogway


ai = oogway.Oogway()
ai.model_name = "gpt-3.5-turbo"
async def query():
    await ai.ask("Why?")
res = query()
print(res)
