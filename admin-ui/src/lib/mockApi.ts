// Mock API handler for development mode
import { AxiosResponse, InternalAxiosRequestConfig } from 'axios';
import {
  mockZone,
  mockStatus,
  mockConnectionStatus,
  mockDnsRecords,
  mockCacheStats,
  mockSecurityLevel,
  mockFirewallRules,
  mockIpAccessRules,
  mockSecurityEvents,
  mockWafRules,
  mockSslStatus,
  mockCertificates,
  mockWorkers,
  mockWorkerRoutes,
  mockKVNamespaces,
  mockKVKeys,
  mockR2Buckets,
  mockR2Objects,
  mockStreamVideos,
  mockStreamLiveInputs,
  mockAnalytics,
  mockBandwidthStats,
  mockSecurityAnalytics,
  mockPageRules,
  mockZoneSettings,
  mockPluginSettings,
  delay,
} from './mockData';

// Helper to create mock response
const mockResponse = <T>(data: T, status = 200): AxiosResponse<{ data: T }> => ({
  data: { data },
  status,
  statusText: 'OK',
  headers: {},
  config: {} as InternalAxiosRequestConfig,
});

// Mock route handlers
type MockHandler = () => Promise<AxiosResponse<any>>;

const mockRoutes: Record<string, Record<string, MockHandler>> = {
  GET: {
    '/cloudflare/status': async () => {
      await delay(300);
      return mockResponse(mockStatus);
    },
    '/cloudflare/zone': async () => {
      await delay(200);
      return mockResponse(mockZone);
    },
    '/cloudflare/connection': async () => {
      await delay(150);
      return mockResponse(mockConnectionStatus);
    },
    '/cloudflare/dns/records': async () => {
      await delay(400);
      return mockResponse(mockDnsRecords);
    },
    '/cloudflare/cache/status': async () => {
      await delay(250);
      return mockResponse(mockCacheStats);
    },
    '/cloudflare/security/level': async () => {
      await delay(150);
      return mockResponse({ level: mockSecurityLevel });
    },
    '/cloudflare/security/firewall/rules': async () => {
      await delay(300);
      return mockResponse(mockFirewallRules);
    },
    '/cloudflare/security/ip-access/rules': async () => {
      await delay(250);
      return mockResponse(mockIpAccessRules);
    },
    '/cloudflare/security/events': async () => {
      await delay(350);
      return mockResponse(mockSecurityEvents);
    },
    '/cloudflare/security/waf/rules': async () => {
      await delay(200);
      return mockResponse(mockWafRules);
    },
    '/cloudflare/ssl/status': async () => {
      await delay(200);
      return mockResponse(mockSslStatus);
    },
    '/cloudflare/ssl/certificates': async () => {
      await delay(300);
      return mockResponse(mockCertificates);
    },
    '/cloudflare/workers': async () => {
      await delay(350);
      return mockResponse(mockWorkers);
    },
    '/cloudflare/workers/routes': async () => {
      await delay(250);
      return mockResponse(mockWorkerRoutes);
    },
    '/cloudflare/workers/kv/namespaces': async () => {
      await delay(300);
      return mockResponse(mockKVNamespaces);
    },
    '/cloudflare/r2/buckets': async () => {
      await delay(350);
      return mockResponse(mockR2Buckets);
    },
    '/cloudflare/stream/videos': async () => {
      await delay(400);
      return mockResponse(mockStreamVideos);
    },
    '/cloudflare/stream/live-inputs': async () => {
      await delay(300);
      return mockResponse(mockStreamLiveInputs);
    },
    '/cloudflare/analytics': async () => {
      await delay(500);
      return mockResponse(mockAnalytics);
    },
    '/cloudflare/analytics/zone': async () => {
      await delay(400);
      return mockResponse(mockAnalytics);
    },
    '/cloudflare/analytics/traffic': async () => {
      await delay(350);
      return mockResponse(mockAnalytics);
    },
    '/cloudflare/analytics/bandwidth': async () => {
      await delay(300);
      return mockResponse(mockBandwidthStats);
    },
    '/cloudflare/analytics/security': async () => {
      await delay(350);
      return mockResponse(mockSecurityAnalytics);
    },
    '/cloudflare/analytics/cache': async () => {
      await delay(300);
      return mockResponse(mockCacheStats);
    },
    '/cloudflare/page-rules': async () => {
      await delay(300);
      return mockResponse(mockPageRules);
    },
    '/cloudflare/zone/settings': async () => {
      await delay(400);
      return mockResponse(mockZoneSettings);
    },
    '/cloudflare/settings': async () => {
      await delay(200);
      return mockResponse(mockPluginSettings);
    },
  },
  POST: {
    '/cloudflare/connection/test': async () => {
      await delay(800);
      return mockResponse({ success: true, message: 'Connection successful' });
    },
    '/cloudflare/cache/purge/all': async () => {
      await delay(600);
      return mockResponse({ success: true, message: 'Cache purged successfully' });
    },
    '/cloudflare/cache/purge': async () => {
      await delay(400);
      return mockResponse({ success: true, message: 'URLs purged successfully' });
    },
    '/cloudflare/cache/purge/tags': async () => {
      await delay(400);
      return mockResponse({ success: true, message: 'Tags purged successfully' });
    },
    '/cloudflare/cache/purge/prefix': async () => {
      await delay(400);
      return mockResponse({ success: true, message: 'Prefix purged successfully' });
    },
    '/cloudflare/cache/warm': async () => {
      await delay(1000);
      return mockResponse({ success: true, message: 'Cache warming started' });
    },
    '/cloudflare/cache/local/clear': async () => {
      await delay(200);
      return mockResponse({ success: true, message: 'Local cache cleared' });
    },
    '/cloudflare/dns/records': async () => {
      await delay(500);
      return mockResponse({ id: `dns_${Date.now()}`, success: true });
    },
    '/cloudflare/dns/import': async () => {
      await delay(800);
      return mockResponse({ success: true, records_imported: 5 });
    },
    '/cloudflare/dns/sync': async () => {
      await delay(600);
      return mockResponse({ success: true, records_synced: 3 });
    },
    '/cloudflare/security/under-attack': async () => {
      await delay(300);
      return mockResponse({ success: true });
    },
    '/cloudflare/security/firewall/rules': async () => {
      await delay(400);
      return mockResponse({ id: `fw_${Date.now()}`, success: true });
    },
    '/cloudflare/security/ip-access/block': async () => {
      await delay(300);
      return mockResponse({ success: true });
    },
    '/cloudflare/security/ip-access/allow': async () => {
      await delay(300);
      return mockResponse({ success: true });
    },
    '/cloudflare/security/ip-access/challenge': async () => {
      await delay(300);
      return mockResponse({ success: true });
    },
    '/cloudflare/ssl/certificates': async () => {
      await delay(800);
      return mockResponse({ id: `cert_${Date.now()}`, success: true });
    },
    '/cloudflare/ssl/certificates/custom': async () => {
      await delay(600);
      return mockResponse({ id: `cert_custom_${Date.now()}`, success: true });
    },
    '/cloudflare/workers': async () => {
      await delay(700);
      return mockResponse({ id: `worker_${Date.now()}`, success: true });
    },
    '/cloudflare/workers/routes': async () => {
      await delay(400);
      return mockResponse({ id: `route_${Date.now()}`, success: true });
    },
    '/cloudflare/workers/templates/deploy': async () => {
      await delay(1000);
      return mockResponse({ success: true, worker_name: 'template-worker' });
    },
    '/cloudflare/workers/kv/namespaces': async () => {
      await delay(500);
      return mockResponse({ id: `kv_${Date.now()}`, success: true });
    },
    '/cloudflare/r2/buckets': async () => {
      await delay(600);
      return mockResponse({ success: true });
    },
    '/cloudflare/page-rules': async () => {
      await delay(400);
      return mockResponse({ id: `rule_${Date.now()}`, success: true });
    },
    '/cloudflare/zone/development-mode': async () => {
      await delay(300);
      return mockResponse({ success: true });
    },
    '/cloudflare/settings/reset': async () => {
      await delay(400);
      return mockResponse({ success: true });
    },
  },
  PUT: {
    '/cloudflare/security/level': async () => {
      await delay(300);
      return mockResponse({ success: true });
    },
    '/cloudflare/ssl/mode': async () => {
      await delay(300);
      return mockResponse({ success: true });
    },
    '/cloudflare/settings': async () => {
      await delay(400);
      return mockResponse({ success: true });
    },
  },
  PATCH: {
    '/cloudflare/zone/settings': async () => {
      await delay(400);
      return mockResponse({ success: true });
    },
  },
  DELETE: {},
};

// Pattern-based route matchers using an array of patterns
interface PatternRoute {
  pattern: RegExp;
  handler: MockHandler;
}

const patternRoutes: Record<string, PatternRoute[]> = {
  GET: [
    {
      pattern: /^\/cloudflare\/workers\/kv\/namespaces\/[^/]+\/keys$/,
      handler: async () => {
        await delay(300);
        return mockResponse(mockKVKeys);
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/kv\/namespaces\/[^/]+\/values\/[^/]+$/,
      handler: async () => {
        await delay(200);
        return mockResponse({ value: 'mock-value-data' });
      },
    },
    {
      pattern: /^\/cloudflare\/r2\/buckets\/[^/]+\/objects$/,
      handler: async () => {
        await delay(400);
        return mockResponse(mockR2Objects);
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse(mockWorkers[0]);
      },
    },
  ],
  PUT: [
    {
      pattern: /^\/cloudflare\/dns\/records\/[^/]+$/,
      handler: async () => {
        await delay(400);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/security\/firewall\/rules\/[^/]+$/,
      handler: async () => {
        await delay(400);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/[^/]+$/,
      handler: async () => {
        await delay(500);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/kv\/namespaces\/[^/]+\/values\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/page-rules\/[^/]+$/,
      handler: async () => {
        await delay(400);
        return mockResponse({ success: true });
      },
    },
  ],
  PATCH: [
    {
      pattern: /^\/cloudflare\/security\/waf\/rules\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
  ],
  DELETE: [
    {
      pattern: /^\/cloudflare\/dns\/records\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/security\/firewall\/rules\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/security\/ip-access\/rules\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/[^/]+$/,
      handler: async () => {
        await delay(400);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/routes\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/kv\/namespaces\/[^/]+$/,
      handler: async () => {
        await delay(400);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/workers\/kv\/namespaces\/[^/]+\/values\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/r2\/buckets\/[^/]+$/,
      handler: async () => {
        await delay(400);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/r2\/buckets\/[^/]+\/objects\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
    {
      pattern: /^\/cloudflare\/page-rules\/[^/]+$/,
      handler: async () => {
        await delay(300);
        return mockResponse({ success: true });
      },
    },
  ],
};

// Find matching mock handler
export const findMockHandler = (method: string, url: string): MockHandler | null => {
  const upperMethod = method.toUpperCase();

  // Check exact routes first
  if (mockRoutes[upperMethod]?.[url]) {
    return mockRoutes[upperMethod][url];
  }

  // Check pattern routes
  const patterns = patternRoutes[upperMethod];
  if (patterns) {
    for (const route of patterns) {
      if (route.pattern.test(url)) {
        return route.handler;
      }
    }
  }

  return null;
};

// Check if mock mode is enabled
export const isMockEnabled = (): boolean => {
  return import.meta.env.DEV;
};
