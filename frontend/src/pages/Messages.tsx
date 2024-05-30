
import { For } from "../components/ui/For"
import { useEffect } from "react"
import { SearchBar } from "../components/SearchBar"
import { ActionIcon } from "../components/Actions"
import { Pagination } from "../components/Pagination"
import { MainLayout } from "../layouts/MainLayout"
import { Message, useMessageStore } from "../store/MessageStore"

export const MessagesPage = () => {

    const messageStore = useMessageStore()

    useEffect(() => { messageStore.getMessages() }, [])

    return (
        <MainLayout>

            <section className="w-full flex flex-row mt-12">
                <SearchBar dataStore={messageStore} variant="messages" />
            </section>

            <section className="w-full flex flex-col gap-4 mt-8">

                <For of={messageStore.dataSplice} render={(item: Message, index: number) => (

                    <div key={index} className="w-full h-20 py-4 border-b border-neutral-300 flex flex-col">

                        <div className="w-full flex flex-row justify-between items-center">
                            <div className="font-bold text-lg">{item.from}</div>
                            <div className="text-sm">{item.date}</div>
                        </div>

                        <div className="w-full flex flex-row items-center">
                            <p className="w-4/5">{item.message}</p>
                            <div className="w-1/5 flex flex-row justify-end gap-4">
                                <ActionIcon variant="info" onClick={() => { }} />
                                <ActionIcon variant="reply" onClick={() => { }} />
                                <ActionIcon variant="delete" onClick={() => { }} />
                            </div>
                        </div>

                    </div>

                )} />

                <Pagination dataStore={messageStore} />


            </section>
        </MainLayout>
    )
}
