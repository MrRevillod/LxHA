
import { For } from "./ui/For"
import { UserTableStore } from "../store/UserStore"
import { ReactNode, useState, useEffect } from "react"

interface Props {
    dataStore: UserTableStore,
    variant: "users" | "instances"
    children: ReactNode,
}

export const Table = ({ variant, dataStore, children }: Props) => {

    const { currentPage, totalPages, setCurrentPage, setSearchTerm } = dataStore
    const [search, setSearch] = useState<string>("")

    useEffect(() => { setSearchTerm(search) }, [search])

    const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(e.target.value)
    }

    const handlePageChange = (newPage: number) => {

        if (newPage < 1 || newPage > totalPages) return

        setCurrentPage(newPage)
    }

    return (

        <div className="w-full flex flex-col mt-20 gap-4 text-neutral-950">

            <div className="w-full flex flex-row justify-between items-center">

                <div className="relative w-1/3 flex flex-row items-center">
                    <input type="text" placeholder={`Search ${variant}`}
                        className="pl-10 p-2 border border-neutral-300 rounded-md w-full"
                        value={search}
                        onChange={handleSearch}
                    />
                    <i className="bi bi-search absolute left-2 top-1/2 transform -translate-y-1/2"></i>
                </div>

                <button className="flex items-center justify-center text-lg w-40 h-12 px-4 rounded-md bg-primary text-white font-semibold">
                    {variant === "users" ? "Create User" : "Create Instance"}
                </button>

            </div>

            <div className={`w-full grid grid-cols-${dataStore?.nColumns} gap-4 py-4 border-b border-neutral-300`}>

                <For of={dataStore?.tableColumns} render={(column: string, index: number) => (
                    <div key={index} className={``}>
                        <h4 className="font-semibold">{column}</h4>
                    </div>
                )} />

            </div>

            <div className="w-full h-full flex flex-col">
                {children}
            </div>

            <div className="w-full flex flex-row gap-8 justify-end items-center mt-4">
                <button className="text-white rounded-full w-10 h-10 p-4 flex items-center justify-center bg-primary" onClick={() => handlePageChange(currentPage - 1)}>
                    <i className="font-semibold bi bi-chevron-left"></i>
                </button>
                <span>{currentPage} / {totalPages}</span>
                <button className="text-white font-bold rounded-full w-10 h-10 p-4 flex items-center justify-center bg-primary" onClick={() => handlePageChange(currentPage + 1)}>
                    <i className="bi bi-chevron-right"></i>
                </button>
            </div>

        </div>
    )
}
