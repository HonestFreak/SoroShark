import React from 'react'
import type { NextPage } from 'next'
import Head from 'next/head'
import CryptoTable from './home'

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>
          SoroShark
        </title>
        <meta
          name="description"
          content="Platform to fund your favorite projects"
        />
        
      </Head>
      <CryptoTable/> 
    </>
  )
}

export default Home
