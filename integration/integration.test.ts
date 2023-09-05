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

const TEST_PROJECT_ID = process.env.TEST_PROJECT_ID || 'e4eae1aad4503db9966a04fd045a7e4d'

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

    it('non-scam origin', async () => {
      let resp: any = await http.get(`${BASE_URL}/${TEST_PROJECT_ID}`)
      let csrfToken = resp.headers["x-csrf-token"]

      resp = await http.post(`${url}`, {'origin': 'http://localhost', 'attestationId': 'some'}, {
        headers: { "x-csrf-token": csrfToken },
      })
      expect(resp.status).toBe(200)
      expect(resp.headers["access-control-allow-origin"]).toBe(undefined)

      resp = await http.options(`${url}/some`);
      expect(resp.headers["access-control-allow-origin"]).toBe("*")

      resp = await http.get(`${url}/some`)
      expect(resp.status).toBe(200)
      expect(resp.data.origin).toBe('http://localhost')
      expect(resp.data.isScam).toBe(false)
      expect(resp.headers["access-control-allow-origin"]).toBe("*")
    })
    
    it('scam origin', async () => {
      let resp: any = await http.get(`${BASE_URL}/${TEST_PROJECT_ID}`)
      let csrfToken = resp.headers["x-csrf-token"]

      resp = await http.post(`${url}`, {'origin': 'https://evilpepecoin.com', 'attestationId': 'evil'}, {
        headers: { "x-csrf-token": csrfToken },
      })
      resp = await http.options(`${url}/evil`);
      resp = await http.get(`${url}/evil`)

      expect(resp.data.isScam).toBe(true)
    })

    it('scam: unknown', async () => {
      let resp: any = await http.get(`${BASE_URL}/${TEST_PROJECT_ID}`)
      let csrfToken = resp.headers["x-csrf-token"]

      resp = await http.post(`${url}`, {'origin': 'https://my-dapp.io', 'attestationId': 'tbd'}, {
        headers: { "x-csrf-token": csrfToken },
      })
      resp = await http.options(`${url}/tbd`);
      resp = await http.get(`${url}/tbd`)

      expect(resp.data.isScam).toBe(null)
    })
  
    it('invalid CSRF token', async () => {
        let csrfToken = 'aaaaaaaaaaaa'

        let resp: any = await http.post(`${url}`, {'origin': 'localhost', 'attestationId': 'some'}, {
          headers: { "x-csrf-token": csrfToken },
        })
        expect(resp.status).toBe(403)
    })

    it('expired CSRF token', async () => {
        let csrfToken = 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODk2NjE2MTl9.PE9QkYso_axaWvwnAUDxth2nBQLNAA0pCDxc8WnfPR8'

        let resp: any = await http.post(`${url}`, {'origin': 'localhost', 'attestationId': 'some'}, {
          headers: { "x-csrf-token": csrfToken },
        })
        expect(resp.status).toBe(403)
    })
  })

  describe('Enclave', () => {
    const url = `${BASE_URL}`

    it('get the enclave', async () => {
      let resp: any = await http.get(`${url}/${TEST_PROJECT_ID}`)

      expect(resp.status).toBe(200)

      let policy = resp.headers["content-security-policy"]
      expect(policy).toBe(`frame-ancestors http://*.localhost http://localhost`)
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

    it('project with Verify disabled', async () => {
      let resp = await http.get(`${url}/0eae574067d750bb4a5e84bb99b5845f`)      
      expect(resp.status).toBe(200)

      let policy = resp.headers["content-security-policy"]
      expect(policy).toBe(undefined)
    })
  })
  describe('index.js', () => {
    const url = `${BASE_URL}`

    it('get index.js', async () => {
      let resp: any = await http.get(`${BASE_URL}/${TEST_PROJECT_ID}`)
      let csrfToken = resp.headers["x-csrf-token"]

      resp = await http.get(`${url}/index.js?token=${csrfToken}`)
      expect(resp.status).toBe(200)
    })

    it('doesn\'t allow invalid `token` parameters', async () => {
      let resp = await http.get(`${url}/index.js?token=<img src onerror=alert(document.domain)>`)
      expect(resp.status).toBe(400)
    })
  })
})
