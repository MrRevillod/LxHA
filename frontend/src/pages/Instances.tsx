
import { MainLayout } from "../layouts/MainLayout"

export const InstancesPage = () => {

    return (

        <MainLayout>

            <div className="w-full h-full flex flex-col gap-4 items-center justify-center text-neutral-950">

                <h1 className="text-5xl font-bold">Instances Page</h1>
                <h2>This only appears if you're a administrator</h2>

            </div>

        </MainLayout>
    )
}
