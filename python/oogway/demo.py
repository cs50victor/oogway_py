import asyncio
import sys
import time
import oogway


ai = oogway.Oogway()
ai.model_name = "gpt-3.5-turbo"

async def talk_to_oogway(question: str):
    print(f"\n> You :\n\t{question}");
    while True:
        print("\n> Oogway : \n\t", end="");
        async for chunk in ai.ask(question):
            sys.stdout.write(chunk)
            sys.stdout.flush()
        question = input("\n> You: \t ")

if __name__ == "__main__":
    asyncio.run(talk_to_oogway("why?"))
