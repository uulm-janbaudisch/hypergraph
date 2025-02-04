pkg load io

args = argv();
arg_print = find(strcmp(args, '--print'));

#file = args{1};

data_partitioning = csv2cell("partitioning/run.csv", 1);
partitioner = length(unique(data_partitioning(:, 2)));

cut_index = 4;
original_index = 6;
split_index = 7;
partitioning_index = 8;

data_metrics = csv2cell("metrics/metrics.csv", 1);

index_clauses = 4;
index_variables = 6;
index_literals = 8;
index_width = 10;
index_density = 12;

instances = data_partitioning(1:partitioner:end, 1);
blocks = 2;
x = 1:length(instances);

# Runtime

runtime = figure();

hold on

for i = 1:partitioner
  original = cell2mat(data_partitioning(i:partitioner:end, original_index));
  split = cell2mat(data_partitioning(i:partitioner:end, split_index));

  name = data_partitioning{i, 2};
  semilogy(x, original, "linewidth", 1, ["x;" name " original;"]);
  semilogy(x, split, "linewidth", 1, ["x;" name " split;"]);
end

xticks(1:1:length(instances));

xlabel("Instance");
ylabel("Runtime in ms");

legend("location", "bestoutside");

grid on
hold off

# Cut set

cutset = figure();

hold on

for i = 1:partitioner
  cut = cell2mat(data_partitioning(i:partitioner:end, cut_index));

  name = data_partitioning{i, 2};
  plot(x, cut, "linewidth", 1, ["x;" name ";"]);
end

xticks(1:1:length(instances));

xlabel("Instance");
ylabel("Cut size");

legend("location", "bestoutside");

grid on
hold off

clauses = metric_plot("Clauses", data_metrics, partitioner, blocks, index_clauses);
variables = metric_plot("Variables", data_metrics, partitioner, blocks, index_variables);
literals = metric_plot("Literals", data_metrics, partitioner, blocks, index_literals);
width = metric_plot("Clause width", data_metrics, partitioner, blocks, index_width);
density = metric_plot("Clause density", data_metrics, partitioner, blocks, index_density);

if (length(arg_print) == 1)
  print(runtime, "runtime", "-dsvg");
  print(cutset, "cutset", "-dsvg");
  print(clauses, "clauses", "-dsvg");
  print(variables, "variables", "-dsvg");
  print(literals, "literals", "-dsvg");
  print(width, "width", "-dsvg");
  print(density, "density", "-dsvg");
end

#print(hf, "plot", "-dpdflatexstandalone");
#system("pdflatex plot");

