import React from 'react';
import { Statistics } from '../simulation-wasm'

type StatType = Omit<typeof Statistics.prototype, 'free'>;


interface HistoryChartProps {
  statsHistory: StatType[];
}

const HistoryChart: React.FC<HistoryChartProps> = ({ statsHistory }) => {
  return (
    <div className="flex flex-col gap-4 p-6 bg-gray-800 text-white rounded-lg shadow-xl w-full flex-grow min-h-0">
      <h2 className="text-2xl font-bold text-center border-b border-gray-600 pb-3 flex-shrink-0">
        History
      </h2>
      {statsHistory.length === 0 ? (
        <p className="text-center text-gray-400 italic">
          No completed generations yet.
        </p>
      ) : (
        <div className="flex flex-col gap-2 overflow-y-auto pr-2">
          {/* Header Row */}
          <div className="grid grid-cols-4 text-sm font-bold text-gray-400 sticky top-0 bg-gray-800 pb-2 border-b border-gray-600">
            <div className="text-center px-2 border-r border-gray-700">Gen</div>
            <div className="text-center px-2 border-r border-gray-700">Min</div>
            <div className="text-center px-2 border-r border-gray-700">Avg</div>
            <div className="text-center px-2">Max</div>
          </div>
          {/* Data Rows */}
          {statsHistory.map((stats, index) => (
            <div key={index} className="grid grid-cols-4 text-sm font-mono py-1 items-center">
              <div className="text-center font-sans font-semibold text-gray-300 px-2 border-r border-gray-700">{index + 1}</div>
              <div className="text-center px-2 border-r border-gray-700">{stats.min_score}</div>
              <div className="text-center px-2 border-r border-gray-700">{parseFloat(stats.avg_score.toFixed(2))}</div>
              <div className="text-center px-2">{stats.max_score}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default HistoryChart;