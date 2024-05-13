
export const DashboardPage = () => {

    return (

        <div className="w-screen h-screen bg-black flex flex-col justify-center items-center">

            <h1 className="text-white">Admin Dashboard (ROLE PROTECTED)</h1>

            <div className="h-5/6 w-11/12 bg-red grid grid-cols-2 grid-rows-2 gap-4 p-4">

                <div className="p-2 bg-white rounded-md"></div>
                <div className="p-2 bg-white rounded-md"></div>
                <div className="p-2 bg-white rounded-md"></div>
                <div className="p-2 bg-white rounded-md"></div>

            </div>
        </div>
    )
}
