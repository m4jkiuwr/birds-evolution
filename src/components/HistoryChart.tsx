import React from 'react';

// Define the type for a single stat object, matching what's in SimCanvas
interface StatType {
  min_score: number;
  avg_score: number;
  max_score: number;
}

interface HistoryChartProps {
  statsHistory: StatType[];
}

const HistoryChart: React.FC<HistoryChartProps> = ({ statsHistory }) => {
  return (
    <div className="flex flex-col gap-4 p-6 bg-gray-800 text-white rounded-lg shadow-xl w-[300px]">
      <h2 className="text-2xl font-bold text-center border-b border-gray-600 pb-3">
        History
      </h2>
      {statsHistory.length === 0 ? (
        <p className="text-center text-gray-400 italic">
          No completed generations yet.
        </p>
      ) : (
        <div className="flex flex-col gap-2 overflow-y-auto max-h-[500px] pr-2">
          {/* Header Row */}
          <div className="grid grid-cols-4 gap-2 text-sm font-bold text-gray-400 sticky top-0 bg-gray-800 pb-2">
            <div className="text-left">Gen</div>
            <div className="text-right">Min</div>
            <div className="text-right">Avg</div>
            <div className="text-right">Max</div>
          </div>
          {/* Data Rows */}
          {statsHistory.map((stats, index) => (
            <div key={index} className="grid grid-cols-4 gap-2 text-sm font-mono">
              <div className="text-left font-sans font-semibold text-gray-300">{index + 1}</div>
              <div className="text-right">{stats.min_score}</div>
              <div className="text-right">{parseFloat(stats.avg_score.toFixed(2))}</div>
              <div className="text-right">{stats.max_score}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default HistoryChart;