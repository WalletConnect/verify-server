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
      let resp: any = await http.get(`${url}/${TEST_PROJECT_ID}`)
      let setCookie = resp.headers["set-cookie"]
      let csrfToken = resp.headers["x-csrf-token"]

      resp = await http.post(`${url}`, {'origin': 'localhost', 'attestationId': 'some'}, {
        headers: { "x-csrf-token": csrfToken, cookie: setCookie },
      })
      expect(resp.status).toBe(200)
      expect(resp.headers["access-control-allow-origin"]).toBe(undefined)

      resp = await http.options(`${url}/some`);
      expect(resp.headers["access-control-allow-origin"]).toBe("*")

      resp = await http.get(`${url}/some`)
      expect(resp.status).toBe(200)
      expect(resp.data.origin).toBe('localhost')
      expect(resp.headers["access-control-allow-origin"]).toBe("*")
    })
  })
  describe('Enclave', () => {
    const url = `${BASE_URL}`

    it('get the enclave', async () => {
      let resp: any = await http.get(`${url}/${TEST_PROJECT_ID}`)

      expect(resp.status).toBe(200)

      let policy = resp.headers["content-security-policy"]
      expect(policy).toBe(`frame-ancestors https://*.walletconnect.com https://walletconnect.com`)
    })

    describe('invalid project ID', () => {
      let errMsg = "Invalid URL: ProjectId should be a hex string 32 chars long"
      it('too short', async () => {
        let resp = await http.get(`${url}/aaaaaaaaaa`)
        expect(resp.status).toBe(400)
        expect(resp.data).toBe(errMsg)
      })
      it('too long', async () => {
        let resp = await http.get(`${url}/3bc51577baa09be45c84b85f13419ae8a`)
        expect(resp.status).toBe(400)
        expect(resp.data).toBe(errMsg)
      })
      it('non-hex chars', async () => {
        let resp = await http.get(`${url}/3bc51577baa09be45c84b85f13419aez`)
        expect(resp.status).toBe(400)
        expect(resp.data).toBe(errMsg)
      })
    })

    it('non-existent project', async () => {
      let resp = await http.get(`${url}/3bc51577baa09be45c84b85f13419ae8`)
      expect(resp.status).toBe(404)
      expect(resp.data).toContain("Project with the provided ID doesn't exist")
    })

    it('project without a verified domain', async () => {
      let resp = await http.get(`${url}/22f5c861aeb01d5928e9f347df79f21b`)      
      expect(resp.status).toBe(200)

      let policy = resp.headers["content-security-policy"]
      expect(policy).toBe(undefined)
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
