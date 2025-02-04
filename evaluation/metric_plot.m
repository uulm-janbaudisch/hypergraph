function handle = metric_plot(metric, data, partitioner, blocks, original_index)
  # The split data is located next to the original column.
  split_index = original_index + 1;

  # Create a new figure.
  handle = figure();

  instances = data(1:partitioner:end, 1);
  x = 1:length(instances);

  # Extract the column containing the original data.
  original = cell2mat(data(1:partitioner:end, original_index));

  # Create a subplot per partitioner.
  for i = 1:partitioner
    # The name of the partitioner.
    name = data{i, 2};

    # Set up the subplot for this partitioner.
    current = subplot(blocks, 1, i);
    set(current, "title", name);

    # Hold while drawing the subplot.
    hold on

    # Plot the original data.
    semilogy(x, original, "linewidth", 1, ["x;Original;"]);

    # Extract the split data.
    split = data(i:partitioner:end, split_index);

    # Plot the split data for each block.
    for i = 1:blocks
      # Extract the split data for the current block.
      split_values = cellfun(@(x) str2num(strsplit(x, ";"){i}), split, "UniformOutput", false);
      split_matrix = cell2mat(split_values);

      # Plot it.
      semilogy(x, split_matrix, "linewidth", 1, ["x;Block " num2str(i) ";"]);
    end

    # Set the x-axis ticks according to the amount of instances.
    xticks(1:1:length(instances));

    xlabel("Instance");
    ylabel(metric);

    legend("location", "bestoutside");

    grid on
    hold off
  end
end

