import axios from 'axios'

const http = axios.create({
  validateStatus: (_status) => true,
})

declare let process: {
  env: {
    JEST_ENV: string,
    TEST_TENANT_ID_APNS: string,
    TEST_PROJECT_ID: string,
  }
}

const BASE_URLS = new Map<string, string>([
  ['prod', 'https://verify.walletconnect.com'],
  ['staging', 'https://staging.verify.walletconnect.com'],
  ['dev', 'https://dev.verify.walletconnect.com'],
  ['local', 'http://localhost:3000'],
])

const ENV = process.env.JEST_ENV;

const TEST_PROJECT_ID = process.env.TEST_PROJECT_ID || '3cbaa32f8fbf3cdcc87d27ca1fa68069'

const BASE_URL = BASE_URLS.get(process.env.JEST_ENV)

describe('verify', () => {
  describe('Health', () => {
    const url = `${BASE_URL}/health`

    it('is healthy', async () => {
      const { status } = await http.get(`${url}`)

      expect(status).toBe(200)
    })
  })
  describe('Attestation', () => {
    const url = `${BASE_URL}/attestation`

    it('can set an attestation', async () => {
      let resp: any = await http.post(`${url}`, {'origin': 'localhost', 'attestationId': 'some'})

      expect(resp.status).toBe(200)

      resp = await http.get(`${url}/some`)

      expect(resp.status).toBe(200)
      expect(resp.data.origin).toBe('localhost')
    })
  })
  describe('Enclave', () => {
    const url = `${BASE_URL}`

    it('get the enclave', async () => {
      let resp: any = await http.get(`${url}/${TEST_PROJECT_ID}`)

      expect(resp.status).toBe(200)

      let policy = resp.headers["content-security-policy"]

      if (ENV === 'prod') {
        expect(policy).toBe("frame-ancestors https://*.walletconnect.com")
      } else {
        let wc = "https://*.walletconnect.com https://walletconnect.com"
        let vercel = "https://*.vercel.app https://vercel.app"
        let localhost = "http://*.localhost http://localhost"
        expect(policy).toBe(`frame-ancestors ${wc} ${wc} ${vercel} ${localhost}`)
      }
    })

    it('non-existent project', async () => {
      let resp = await http.get(`${url}/3bc51577baa09be45c84b85f13419ae8`)
      expect(resp.status).toBe(404)
      expect(resp.data).toContain("Project with the provided ID doesn't exist")
    })

    it('project without a verified domain', async () => {
      let resp = await http.get(`${url}/22f5c861aeb01d5928e9f347df79f21b`)

      if (ENV === 'prod') {
        expect(resp.status).toBe(404)
        expect(resp.data).toContain("Project with the provided ID doesn't have a verified domain")
      } else {
        let policy = resp.headers["content-security-policy"]

        let wc = "https://*.walletconnect.com https://walletconnect.com"
        let vercel = "https://*.vercel.app https://vercel.app"
        let localhost = "http://*.localhost http://localhost"
        expect(policy).toBe(`frame-ancestors ${wc} ${vercel} ${localhost}`)
      }
    })
  })
  describe('index.js', () => {
    const url = `${BASE_URL}`

    it('get index.js', async () => {
      let resp: any = await http.get(`${url}/index.js`)
      expect(resp.status).toBe(200)
    })
  })
})
