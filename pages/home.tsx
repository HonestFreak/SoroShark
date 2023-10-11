import React, { Component } from 'react';
import styles from '../styles/Home.module.css'
import { WalletData } from '../components/molecules'
class CryptoTable extends Component {
  constructor(props) {
    super(props);
    this.state = {
      searchQuery: '',
      cryptoData: [
        {
          name: 'MyStartup',
          symbol: 'MCP1',
          price: 48000,
          marketCap: 10000,
          tokenized: 20,
        },
        {
          name: 'SampleCompany2',
          symbol: 'SCP2',
          price: 3500,
          marketCap: 85000,
          tokenized: 5,
        },
        {
          name: 'Web3Startup',
          symbol: 'W3S',
          price: 2.5,
          marketCap: 50000,
          tokenized: 10,
        },
        {
          name: 'SocialBlockchain',
          symbol: 'SCB',
          price: 420,
          marketCap: 234000,
          tokenized: 40,
        },
        {
          name: 'DLib',
          symbol: 'DLIB',
          price: 160,
          marketCap: 4500,
          tokenized: 50,
        },
        {
          name: 'EduVerse',
          symbol: 'EDV',
          price: 1.25,
          marketCap: 500,
          tokenized: 10,
        },
        {
          name: 'PDStartup',
          symbol: 'PDS',
          price: 35,
          marketCap: 320000,
          tokenized: 20,
        },
        {
          name: 'NewStartup',
          symbol: 'NSS',
          price: 0.3,
          marketCap: 4000,
          tokenized: 2,
        },
        {
          name: 'SampleCompanyNew',
          symbol: 'SC1',
          price: 70,
          marketCap: 18000,
          tokenized: 23,
        },
        {
          name: 'AI Startup',
          symbol: 'AIS',
          price: 30,
          marketCap: 130000,
          tokenized: 20,
        },
      ],
    };
  }

  handleSearchChange = (event) => {
    this.setState({ searchQuery: event.target.value });
  };

  handleTokenizeEquityClick = () => {
    // Placeholder for tokenization logic
  };

  render() {
    const { searchQuery, cryptoData } = this.state;
    const filteredCryptoData = cryptoData.filter((crypto) =>
      crypto.name.toLowerCase().includes(searchQuery.toLowerCase())
    );

    return (
        <>  <header className={styles.header}>
        <h3 style={{color:"white"}}> 🐋 SoroShark</h3> 
        <div className="search-bar">
          <input
            type="text"
            placeholder="Search Crypto..."
            value={searchQuery}
            onChange={this.handleSearchChange}
          />
        </div>
        <button className="tokenize-button">
          Tokenize Equity
        </button>
        <WalletData />
      </header> 
      <br/>

      <div className="crypto-table-container">  
        <table className="crypto-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Symbol</th>
              <th>Price</th>
              <th>Valuation</th>
              <th>Equity Tokenized</th>
              <th>View Details</th>
            </tr>
          </thead>
          <tbody>
            {filteredCryptoData.map((crypto, index) => (
              <tr key={index}>
                <td>{crypto.name}</td>
                <td>{crypto.symbol}</td>
                <td>${crypto.price}</td>
                <td>${crypto.marketCap.toLocaleString()}</td>
                <td>{crypto.tokenized}%</td>
                <td><button className='viewdetail'> View Details</button> </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div> </>
    );
  }
}

export default CryptoTable;
