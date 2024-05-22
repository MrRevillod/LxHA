
import "../index.css"

import { LandingHeader } from "../components/Header"

export const LandingPage = () => {

    return (

        <main className="h-screen w-screen flex flex-col px-6 md:px-12 lg:px-28 py-8">

            <LandingHeader />

            <article className="h-full w-full flex flex-row gap-4 items-center">

                <section className="w-1/2 flex flex-col gap-8">

                    <h1 className="text-7xl ">LX <br /> High Availability</h1>

                    <p className="w-5/6">
                        Leveraging LXD technology, we ensure continuous service
                        with automated failover. Easily create, deploy, and monitor
                        instances while optimizing resources. Ideal for web hosting,
                        development, testing, and critical applications. Enjoy reliability,
                        efficiency, security, scalability, and cost-effectiveness. Explore
                        how our software can benefit your organization.
                    </p>

                    <a href="https://linuxcontainers.org/" target="_blank" className="hover:text-blue-800 underline">
                        Linux containers documentation
                    </a>

                </section>

                <section className="w-1/2 h-1/3 landing-img-container">

                    <div className="behind">
                        <img src="/landing-img-1.png" alt="" className="w-5/6" />
                    </div>
                    <div className="over">
                        <img src="/landing-img-2.png" alt="" className="w-5/6" />
                    </div>

                </section>

            </article>

        </main>
    )
}