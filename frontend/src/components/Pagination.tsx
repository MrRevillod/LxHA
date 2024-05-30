
import { UserStore } from "../store/UserStore"
import { MessageStore } from "../store/MessageStore"
import { InstanceStore } from "../store/InstanceStore"
import { useEffect, useState } from "react"

interface PaginationProps {
    dataStore: MessageStore | UserStore | InstanceStore
}

export const Pagination = ({ dataStore }: PaginationProps) => {

    const { filteredData, itemsPerPage, setSplicedData } = dataStore

    const [currentPage, setCurrentPage] = useState<number>(1)
    const [totalPages, setTotalPages] = useState<number>(Math.ceil(filteredData.length / itemsPerPage))

    const handlePageChange = (page: number) => {

        if (page < 1 || page > totalPages) return

        setCurrentPage(page)
    }

    useEffect(() => {

        setTotalPages(Math.ceil(filteredData.length / itemsPerPage))

    }, [dataStore])

    useEffect(() => { setSplicedData(currentPage) }, [currentPage, setSplicedData])

    return (

        <div className="flex flex-row gap-8 justify-end items-center mt-4 fixed bottom-6 md:bottom-10 right-10 2xl:right-28 -mr-1">

            <button
                className="text-white rounded-full w-10 h-10 p-4 flex items-center justify-center bg-primary"
                onClick={() => handlePageChange(currentPage - 1)}
            >
                <i className="font-semibold bi bi-chevron-left"></i>
            </button>

            <span>{currentPage} / {totalPages}</span>

            <button
                className="text-white font-bold rounded-full w-10 h-10 p-4 flex items-center justify-center bg-primary"
                onClick={() => handlePageChange(currentPage + 1)}
            >
                <i className="bi bi-chevron-right"></i>
            </button>

        </div>
    )
}