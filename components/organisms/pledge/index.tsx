import React, { FunctionComponent, useState } from 'react'
import { Card, ConnectButton, Loading, ProgressBar } from '../../atoms'
import styles from './style.module.css'
import { Spacer } from '../../atoms/spacer'
import { Utils } from '../../../shared/utils'
import {
  useAccount,
  useSubscription,
} from '../../../hooks'
import {
  // crowdfund as crowdfundContract,
  equity as equityContract,
} from '../../../shared/contracts'

import * as SorobanClient from 'soroban-client'
import { Deposits, FormPledge } from '../../molecules'
let xdr = SorobanClient.xdr

const Pledge: FunctionComponent = () => {
  const [updatedAt, setUpdatedAt] = React.useState<number>(Date.now())
  const account = useAccount()

  const [equity, setequity] = React.useState<{
    balance: BigInt
    decimals: number
    name: string
    symbol: string
  }>()

  // const [crowdfund, setCrowdfund] = React.useState<{
  //   deadline: Date
  //   target: BigInt
  // }>()

  React.useEffect(() => {
    Promise.all([
      // equityContract.balance({ id: crowdfundContract.options.contractId }),
      equityContract.decimals(),
      equityContract.name(),
      equityContract.symbol(),

      // crowdfundContract.deadline(),
      // crowdfundContract.target(),
    ]).then(fetched => {
      setequity({
        balance: fetched[0],
        decimals: fetched[1],
        name: fetched[2].toString(),
        symbol: fetched[3].toString(),
      })

      // setCrowdfund({
      //   deadline: new Date(Number(fetched[4]) * 1000),
      //   target: fetched[5],
      // })
    })
  }, [updatedAt])

  const [targetReached, setTargetReached] = useState<boolean>(false)

  // useSubscription(
  //   crowdfundContract.options.contractId,
  //   'pledged_amount_changed',
  //   React.useMemo(() => event => {
  //     let eventTokenBalance = xdr.ScVal.fromXDR(event.value.xdr, 'base64')
  //     setequity({ ...equity!, balance: SorobanClient.scValToNative(eventTokenBalance) })
  //   }, [equity])
  // )

  // useSubscription(
  //   crowdfundContract.options.contractId,
  //   'target_reached',
  //   React.useMemo(() => () => {
  //     setTargetReached(true)
  //   }, [])
  // )

  return (
    <Card>
      {!equity || !crowdfund ? (
        <Loading size={64} />
      ) : (
        <>
          {targetReached && (
            <h6>SUCCESSFUL CAMPAIGN !!</h6>
          )}
          <h6>PLEDGED</h6>
          <div className={styles.pledgeAmount}>
            {Utils.formatAmount(equity.balance, equity.decimals)} {equity.symbol}
          </div>
          <span className={styles.pledgeGoal}>{`of ${Utils.formatAmount(
            crowdfund.target,
            equity.decimals
          )} ${equity.symbol} goal`}</span>
          <ProgressBar
            value={Utils.percentage(equity.balance, crowdfund.target, equity.decimals)}
          />
          <div className={styles.wrapper}>
            <div>
              <h6>Time remaining</h6>
              <span className={styles.values}>
                {Utils.getRemainingTime(crowdfund.deadline)}
              </span>
            </div>
            <div>
              <h6>Backers</h6>
              <span className={styles.values}>976</span>
            </div>
          </div>
          <Spacer rem={1.5} />
          {!Utils.isExpired(crowdfund.deadline) &&
            (account ? (
              <FormPledge
                decimals={equity.decimals || 7}
                account={account.address}
                symbol={equity.symbol}
                updatedAt={updatedAt}
                onPledge={() => setUpdatedAt(Date.now())}
              />
            ) : (
              <ConnectButton label="Connect wallet to pledge" isHigher={true} />
            ))}
          {account && (
            <Deposits
              address={account.address}
              decimals={equity.decimals || 7}
              name={equity.name}
              symbol={equity.symbol}
            />
          )}
        </>
      )}
    </Card>
  )
}

export { Pledge }
