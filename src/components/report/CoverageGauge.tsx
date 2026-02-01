import { PieChart, Pie, Cell, ResponsiveContainer } from "recharts";

interface Props {
  coveragePercent: number;
  total: number;
  covered: number;
}

export function CoverageGauge({ coveragePercent, total, covered }: Props) {
  const data = [
    { name: "Covered", value: covered },
    { name: "Uncovered", value: total - covered },
  ];

  const color = coveragePercent >= 80 ? "#22c55e" : coveragePercent >= 50 ? "#eab308" : "#ef4444";

  return (
    <div className="flex items-center gap-4">
      <div className="w-24 h-24 relative">
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              data={data}
              cx="50%"
              cy="50%"
              innerRadius={28}
              outerRadius={40}
              startAngle={90}
              endAngle={-270}
              dataKey="value"
              stroke="none"
            >
              <Cell fill={color} />
              <Cell fill="#333348" />
            </Pie>
          </PieChart>
        </ResponsiveContainer>
        <div className="absolute inset-0 flex items-center justify-center">
          <span className="text-lg font-bold" style={{ color }}>
            {coveragePercent.toFixed(0)}%
          </span>
        </div>
      </div>
      <div>
        <p className="text-sm text-text">{covered}/{total} requirements covered</p>
        <p className="text-xs text-text-muted mt-0.5">
          {total - covered} uncovered
        </p>
      </div>
    </div>
  );
}
