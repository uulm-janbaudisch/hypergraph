pkg load io

args = argv();

data = csv2cell(args{1}, 1);
partitioner = length(unique(data(:, 2)));

cut_index = 4;
diff_index = 6;

x = 1:length(data(1:partitioner:end, 1));

subplot (2, 1, 1)

hold on

for i = 1:partitioner
  cut = cell2mat(data(i:partitioner:end, cut_index));

  name = data{i, 2};
  plot(x, cut, "linewidth", 2, [":x;" name ";"]);
end

xlabel("Instance");
ylabel("Cut size");

hold off

subplot (2, 1, 2)

hold on

for i = 1:partitioner
  diff = cell2mat(data(i:partitioner:end, diff_index));

  name = data{i, 2};
  plot(x, diff, "linewidth", 2, [":x;" name ";"]);
end

xlabel("Instance");
ylabel("Runtime difference to d4 in s");

hold off

print -dpng plot.png

