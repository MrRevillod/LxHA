
import { For } from "../components/ui/For"
import { Helmet } from "react-helmet"
import { Message } from "../lib/types"
import { useEffect } from "react"
import { SearchBar } from "../components/SearchBar"
import { Pagination } from "../components/Pagination"
import { MainLayout } from "../layouts/MainLayout"
import { useMessageStore } from "../store/MessageStore"

import "../index.css"
import { toSuspensive } from "../lib/string"

export const MessagesPage = () => {

    const messageStore = useMessageStore()

    useEffect(() => { messageStore.getMessages() }, [])

    return (

        <MainLayout>

            <Helmet>
                <title>Lx High Availability - Messages</title>
            </Helmet>

            <section className="w-full flex flex-row mt-12">
                <SearchBar dataStore={messageStore} variant="messages" />
            </section>

            <section className="w-full flex flex-col gap-4 mt-8">

                <For of={messageStore.dataSplice} render={(item: Message, index: number) => (

                    <div key={index} className="gap-1 max-w-full w-full border border-neutral-200 flex flex-col items-center rounded-md px-4 py-2 hover:bg-gray-100 transition-colors">

                        <div className="flex flex-row justify-between w-full">

                            <div className="flex items-center gap-2">
                                <div className="font-medium text-gray-900 truncate">{item.from}</div>
                                <div className="text-xs text-gray-500 truncate">{item.subject}</div>
                            </div>

                            <div className="font-medium text-gray-900">{item.date}</div>

                        </div>

                        <div className="flex items-center w-full overflow-hidden">
                            <p className=" text-gray-500 truncate">{toSuspensive(item.message, 105)}</p>
                        </div>

                    </div>
                )} />

                <Pagination dataStore={messageStore} />

            </section>

        </MainLayout>
    )
}
