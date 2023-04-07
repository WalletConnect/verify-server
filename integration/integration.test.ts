import axios from 'axios'

declare let process: {
  env: {
    JEST_ENV: string,
    TEST_TENANT_ID_APNS: string,
  }
}

const BASE_URLS = new Map<string, string>([
  ['prod', 'https://verify.walletconnect.com'],
  ['staging', 'https://staging.verify.walletconnect.com'],
  ['dev', 'https://dev.verify.walletconnect.com'],
  ['local', 'http://localhost:3000'],
])

const TEST_TENANT = process.env.TEST_TENANT_ID_APNS

const BASE_URL = BASE_URLS.get(process.env.JEST_ENV)

describe('verify', () => {
  describe('Health', () => {
    const url = `${BASE_URL}/health`

    it('is healthy', async () => {
      const { status } = await axios.get(`${url}`)

      expect(status).toBe(200)
    })
  })
  describe('Attestation', () => {
    const url = `${BASE_URL}/attestation`

    it('can set an attestation', async () => {
      let resp: any = await axios.post(`${url}`, {'origin': 'localhost', 'attestationId': 'some'})

      expect(resp.status).toBe(200)

      resp = await axios.get(`${url}/some`)

      expect(resp.status).toBe(200)
      console.log('headers', Object.keys(resp.headers))
      expect(resp.data.origin).toBe('localhost')
    })
  })
  describe('Enclave', () => {
    const url = `${BASE_URL}`

    it('get the enclave', async () => {
      let resp: any = await axios.get(`${url}/someProjectId`)

      expect(resp.status).toBe(200)

      let policy = resp.headers["content-security-policy"]
      expect(policy).toMatch(new RegExp("^frame-ancestors"))
      expect(policy).toContain("https://react-app.walletconnect.com")
    })
  })
  describe('index.js', () => {
    const url = `${BASE_URL}`

    it('get index.js', async () => {
      let resp: any = await axios.get(`${url}/index.js`)

      expect(resp.status).toBe(200)

      let policy = resp.headers["content-security-policy"]
      expect(policy).toMatch(new RegExp("^frame-ancestors"))
      expect(policy).toContain("https://react-app.walletconnect.com")
    })
  })
})
