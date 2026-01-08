import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import {
  Video,
  Upload,
  Play,
  Pause,
  Trash2,
  Copy,
  ExternalLink,
  RefreshCw,
  Radio,
  Settings,
  Eye,
  Clock,
  HardDrive,
  Users,
} from 'lucide-react';
import { cloudflareApi } from '../lib/api';
import toast from 'react-hot-toast';
import { clsx } from 'clsx';
import { StatsSkeleton, CardSkeleton } from '../components/Skeleton';

interface StreamVideo {
  uid: string;
  thumbnail: string;
  readyToStream: boolean;
  status: { state: string };
  meta: { name: string };
  created: string;
  duration: number;
  size: number;
  playback: { hls: string; dash: string };
  preview: string;
}

const formatDuration = (seconds: number): string => {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
};

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

export function StreamPage() {
  const queryClient = useQueryClient();
  const [showUploadModal, setShowUploadModal] = useState(false);
  const [showLiveModal, setShowLiveModal] = useState(false);

  // Note: These would need corresponding backend endpoints
  const { data: videos, isLoading } = useQuery({
    queryKey: ['stream-videos'],
    queryFn: async () => {
      // Placeholder - would call cloudflareApi.listStreamVideos()
      return { data: { data: [] } };
    },
  });

  const { data: liveInputs } = useQuery({
    queryKey: ['stream-live-inputs'],
    queryFn: async () => {
      // Placeholder - would call cloudflareApi.listLiveInputs()
      return { data: { data: [] } };
    },
  });

  const videosList: StreamVideo[] = videos?.data?.data || [];
  const liveInputsList: any[] = liveInputs?.data?.data || [];

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    toast.success('Copied to clipboard');
  };

  return (
    <div className="p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold text-neutral-100">Stream</h1>
          <p className="text-neutral-400">
            Video hosting and live streaming at the edge
          </p>
        </div>
        <div className="flex gap-3">
          <button
            onClick={() => setShowLiveModal(true)}
            className="flex items-center gap-2 px-4 py-2 border border-neutral-600 rounded-lg hover:bg-neutral-700/50 text-neutral-300"
          >
            <Radio className="w-4 h-4 text-red-500" />
            Create Live Input
          </button>
          <button
            onClick={() => setShowUploadModal(true)}
            className="flex items-center gap-2 px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg"
          >
            <Upload className="w-4 h-4" />
            Upload Video
          </button>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
        <div className="bg-neutral-800 rounded-xl border border-neutral-700/50 p-4">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-orange-500/10 rounded-lg">
              <Video className="w-5 h-5 text-orange-500" />
            </div>
            <div>
              <p className="text-sm text-neutral-400">Total Videos</p>
              <p className="text-xl font-bold text-neutral-100">{videosList.length}</p>
            </div>
          </div>
        </div>
        <div className="bg-neutral-800 rounded-xl border border-neutral-700/50 p-4">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-red-500/10 rounded-lg">
              <Radio className="w-5 h-5 text-red-500" />
            </div>
            <div>
              <p className="text-sm text-neutral-400">Live Inputs</p>
              <p className="text-xl font-bold text-neutral-100">{liveInputsList.length}</p>
            </div>
          </div>
        </div>
        <div className="bg-neutral-800 rounded-xl border border-neutral-700/50 p-4">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-blue-500/10 rounded-lg">
              <Eye className="w-5 h-5 text-blue-500" />
            </div>
            <div>
              <p className="text-sm text-neutral-400">Total Views</p>
              <p className="text-xl font-bold text-neutral-100">0</p>
            </div>
          </div>
        </div>
        <div className="bg-neutral-800 rounded-xl border border-neutral-700/50 p-4">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-green-500/10 rounded-lg">
              <HardDrive className="w-5 h-5 text-green-500" />
            </div>
            <div>
              <p className="text-sm text-neutral-400">Storage Used</p>
              <p className="text-xl font-bold text-neutral-100">0 GB</p>
            </div>
          </div>
        </div>
      </div>

      {/* Videos Grid */}
      <div className="bg-neutral-800 rounded-xl border border-neutral-700/50">
        <div className="px-6 py-4 border-b border-neutral-700/50">
          <h2 className="text-lg font-semibold text-neutral-100">Videos</h2>
        </div>

        {isLoading ? (
          <div className="p-6">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {[1, 2, 3].map((i) => (
                <CardSkeleton key={i} />
              ))}
            </div>
          </div>
        ) : videosList.length === 0 ? (
          <div className="p-12 text-center text-neutral-500">
            <Video className="w-16 h-16 mx-auto mb-4 opacity-50" />
            <p className="text-lg font-medium mb-2 text-neutral-300">No videos uploaded</p>
            <p className="text-sm mb-4">Upload your first video to get started with Cloudflare Stream</p>
            <button
              onClick={() => setShowUploadModal(true)}
              className="px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg"
            >
              Upload Video
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-6">
            {videosList.map((video) => (
              <div
                key={video.uid}
                className="border border-neutral-700/50 rounded-lg overflow-hidden hover:border-neutral-600 transition-colors"
              >
                <div className="relative aspect-video bg-neutral-900">
                  {video.thumbnail ? (
                    <img
                      src={video.thumbnail}
                      alt={video.meta.name}
                      className="w-full h-full object-cover"
                    />
                  ) : (
                    <div className="w-full h-full flex items-center justify-center">
                      <Video className="w-12 h-12 text-neutral-600" />
                    </div>
                  )}
                  <div className="absolute bottom-2 right-2 px-2 py-1 bg-black/70 text-white text-xs rounded">
                    {formatDuration(video.duration)}
                  </div>
                  {!video.readyToStream && (
                    <div className="absolute inset-0 bg-black/50 flex items-center justify-center">
                      <div className="text-center text-white">
                        <RefreshCw className="w-8 h-8 animate-spin mx-auto mb-2" />
                        <p className="text-sm">Processing...</p>
                      </div>
                    </div>
                  )}
                </div>
                <div className="p-4 bg-neutral-800/50">
                  <h3 className="font-medium text-neutral-100 truncate">
                    {video.meta.name}
                  </h3>
                  <div className="flex items-center gap-4 mt-2 text-sm text-neutral-500">
                    <span className="flex items-center gap-1">
                      <Clock className="w-3 h-3" />
                      {new Date(video.created).toLocaleDateString()}
                    </span>
                    <span>{formatBytes(video.size)}</span>
                  </div>
                  <div className="flex items-center gap-2 mt-4">
                    <button
                      onClick={() => window.open(video.preview, '_blank')}
                      className="flex-1 flex items-center justify-center gap-1 px-3 py-1.5 text-sm border border-neutral-600 rounded hover:bg-neutral-700/50 text-neutral-300"
                    >
                      <Play className="w-3 h-3" />
                      Preview
                    </button>
                    <button
                      onClick={() => copyToClipboard(video.playback.hls)}
                      className="p-1.5 text-neutral-400 hover:text-neutral-200 border border-neutral-600 rounded"
                    >
                      <Copy className="w-4 h-4" />
                    </button>
                    <button className="p-1.5 text-neutral-400 hover:text-red-500 border border-neutral-600 rounded">
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Live Inputs Section */}
      <div className="mt-8 bg-neutral-800 rounded-xl border border-neutral-700/50">
        <div className="px-6 py-4 border-b border-neutral-700/50">
          <h2 className="text-lg font-semibold text-neutral-100 flex items-center gap-2">
            <Radio className="w-5 h-5 text-red-500" />
            Live Inputs
          </h2>
        </div>

        {liveInputsList.length === 0 ? (
          <div className="p-12 text-center text-neutral-500">
            <Radio className="w-16 h-16 mx-auto mb-4 opacity-50" />
            <p className="text-lg font-medium mb-2 text-neutral-300">No live inputs configured</p>
            <p className="text-sm mb-4">Create a live input to start streaming</p>
            <button
              onClick={() => setShowLiveModal(true)}
              className="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg"
            >
              Create Live Input
            </button>
          </div>
        ) : (
          <div className="p-6">
            <table className="w-full">
              <thead>
                <tr className="text-left text-xs font-medium text-neutral-500 uppercase">
                  <th className="pb-3">Name</th>
                  <th className="pb-3">Status</th>
                  <th className="pb-3">RTMP URL</th>
                  <th className="pb-3">Created</th>
                  <th className="pb-3 text-right">Actions</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-neutral-700/50">
                {liveInputsList.map((input: any) => (
                  <tr key={input.uid}>
                    <td className="py-4 font-medium text-neutral-100">{input.meta?.name || 'Unnamed'}</td>
                    <td className="py-4">
                      <span className={clsx(
                        'px-2 py-1 text-xs rounded-full',
                        input.status?.current?.state === 'connected'
                          ? 'bg-green-500/10 text-green-400 border border-green-500/20'
                          : 'bg-neutral-700/50 text-neutral-400 border border-neutral-600'
                      )}>
                        {input.status?.current?.state || 'idle'}
                      </span>
                    </td>
                    <td className="py-4">
                      <code className="text-xs bg-neutral-700/50 px-2 py-1 rounded text-neutral-300">
                        rtmps://...
                      </code>
                    </td>
                    <td className="py-4 text-sm text-neutral-500">
                      {new Date(input.created).toLocaleDateString()}
                    </td>
                    <td className="py-4 text-right">
                      <button className="p-1 text-neutral-500 hover:text-primary-400 mr-2">
                        <Settings className="w-4 h-4" />
                      </button>
                      <button className="p-1 text-neutral-500 hover:text-red-500">
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Feature Info */}
      <div className="mt-8 bg-gradient-to-r from-orange-500 to-red-500 rounded-lg p-6 text-white">
        <h3 className="text-xl font-bold mb-2">Cloudflare Stream</h3>
        <p className="text-orange-100 mb-4">
          Stream video on-demand and live with automatic encoding, adaptive bitrate streaming, and global delivery.
        </p>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="flex items-start gap-3">
            <div className="p-2 bg-white/20 rounded-lg">
              <Video className="w-5 h-5" />
            </div>
            <div>
              <p className="font-medium">Automatic Encoding</p>
              <p className="text-sm text-orange-100">Videos are automatically transcoded for optimal playback</p>
            </div>
          </div>
          <div className="flex items-start gap-3">
            <div className="p-2 bg-white/20 rounded-lg">
              <Radio className="w-5 h-5" />
            </div>
            <div>
              <p className="font-medium">Live Streaming</p>
              <p className="text-sm text-orange-100">Low-latency live streaming with instant rewind</p>
            </div>
          </div>
          <div className="flex items-start gap-3">
            <div className="p-2 bg-white/20 rounded-lg">
              <Users className="w-5 h-5" />
            </div>
            <div>
              <p className="font-medium">Global Delivery</p>
              <p className="text-sm text-orange-100">Stream to viewers worldwide with edge caching</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
